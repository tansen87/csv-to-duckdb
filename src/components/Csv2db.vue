<template>
  <div
    class="q-pa-md"
    style="
      display: flex;
      justify-content: space-between;
      max-width: 1500px;
      margin: 0 auto;
    "
  >
    <!-- column 1 -->
    <div class="q-gutter-y-md" style="flex-basis: 23%; max-width: 350px">
      <q-input color="teal" v-model="data.table" label="table name">
        <template v-slot:prepend>
          <q-icon name="south_east" />
          <q-select
            v-model="data.sep"
            :options="sepOptions"
            label="sep"
            style="width: 58px"
          />
        </template>
      </q-input>

      <q-input
        v-model="data.quote"
        :options="sepOptions"
        label="quote"
        style="width: 50px"
      />
    </div>

    <!-- column 2 -->
    <div class="q-gutter-y-md" style="flex-basis: 33%; max-width: 500px">
      <q-input color="teal" v-model="data.file" label="csv file" autogrow />
    </div>

    <!-- column 3 -->
    <div class="q-gutter-y-md" style="flex-basis: 23%; max-width: 290px">
      <q-input>
        <template v-slot:prepend>
          <q-btn
            color="secondary"
            label="open file"
            @click="openFile"
            style="width: 145px"
          />
          <q-btn
            color="secondary"
            label="write"
            @click="writeDB"
            style="width: 145px"
          />
        </template>
      </q-input>
    </div>
  </div>

  <div
    class="q-pa-md"
    style="
      display: flex;
      justify-content: space-between;
      max-width: 1500px;
      margin: 0 auto;
    "
  >
    <q-table
      title="csv view"
      :rows="tableData"
      :columns="columns"
      row-key="name"
      style="height: 400px; width: 100%"
    >
    </q-table>
  </div>
</template>

<script setup>
import { ref, reactive } from "vue";
import { useQuasar, QSpinnerGears, Notify } from "quasar";
import { open, save } from "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

const $q = useQuasar();

const columns = ref([]);
const tableData = ref([]);
const data = reactive({
  table: "t1",
  file: "",
  fileExtension: ["*"],
  sep: ",",
  quote: "",
});
const sepOptions = [",", "|", "\\t", ";"];

listen("csv2json_err", (event) => {
  const error = "csv2json_err: " + event.payload;
  Notify.create({
    type: "negative",
    message: error,
    position: "bottom-right",
  });
});

// open file
async function openFile() {
  const selected = await open({
    multiple: false,
    filters: [
      {
        name: "CSV",
        extensions: data.fileExtension,
      },
    ],
  });
  if (Array.isArray(selected)) {
    data.file = selected.toString();
  } else if (selected === null) {
    Notify.create({
      type: "warning",
      message: "未选择csv文件",
      position: "bottom-right",
    });
    return;
  } else {
    data.file = selected;
  }

  const result = await invoke("view", {
    file: data.file,
    sep: data.sep,
  });
  const jsonData = JSON.parse(result);
  columns.value = Object.keys(jsonData[0]).map((key) => ({
    name: key,
    label: key,
    field: key,
  }));
  tableData.value = jsonData;
}

// write csv to duckdb
async function writeDB() {
  if (data.file === "") {
    Notify.create({
      type: "warning",
      message: "未选择csv文件",
      position: "bottom-right",
    });
    return;
  }

  const outputPath = await save({
    title: "Export",
    defaultPath: `filename.duckdb`,
    filters: [{ name: "duckdb", extensions: ["duckdb"] }],
  });
  if (outputPath === "" || outputPath === null) {
    Notify.create({
      type: "warning",
      message: "未选择保存文件",
      position: "bottom-right",
    });
    return;
  }

  const notif = $q.notify({
    color: "ongoing",
    icon: "ongoing",
    position: "bottom-right",
    group: false,
    timeout: 0,
    spinner: QSpinnerGears,
    message: "正在写入duckdb...",
  });

  const res = await invoke("csv2db", {
    table: data.table,
    file: data.file,
    sep: data.sep,
    quote: data.quote,
    outputPath: outputPath,
  });

  if (res.includes("Error")) {
    let notifId = notif({
      message: res,
      color: "negative",
      icon: "sentiment_very_dissatisfied",
      spinner: false,
      actions: [
        {
          label: "ok",
          color: "white",
          handler: () => {
            notifId = null;
          },
        },
      ],
    });
  } else {
    let notifId = notif({
      message: "写入成功,耗时: " + res,
      color: "positive",
      icon: "sentiment_satisfied_alt",
      spinner: false,
      actions: [
        {
          label: "ok",
          color: "white",
          handler: () => {
            notifId = null;
          },
        },
      ],
    });
  }
}
</script>
