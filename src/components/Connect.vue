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
      <q-input color="teal" v-model="data.table" label="Table">
        <template v-slot:prepend>
          <q-icon name="south_east" />
          <q-select
            v-model="data.sep"
            :options="sepOptions"
            label="Separator"
            style="width: 90px"
          />
        </template>
      </q-input>
    </div>

    <!-- column 2 -->
    <div class="q-gutter-y-md" style="flex-basis: 23%; max-width: 350px">
      <q-input color="teal" v-model="data.file" label="File"> </q-input>
      <q-input color="teal" v-model="data.dbFile" label="Database"> </q-input>
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
      <q-input>
        <template v-slot:prepend>
          <q-btn
            color="secondary"
            label="open db"
            @click="openDBfile"
            style="width: 145px"
          />
          <q-btn
            color="secondary"
            label="query"
            @click="query"
            style="width: 145px"
          />
        </template>
      </q-input>
    </div>

    <!-- column 4 -->
    <div class="q-gutter-y-md" style="flex-basis: 23%; max-width: 350px"></div>
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
    <div class="q-gutter-y-md" style="flex-basis: 80%; max-width: 1200px">
      <q-input v-model="data.sql" color="teal" label="SQL" autogrow />
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
      style="height: 430px; width: 100%"
    >
    </q-table>
  </div>
</template>

<script setup>
import { ref, reactive } from "vue";
import { useQuasar, QSpinnerGears, Notify } from "quasar";
import { open } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";

const $q = useQuasar();

const columns = ref([]);
const tableData = ref([]);
const data = reactive({
  table: "duck",
  file: "",
  fileExtension: ["csv", "txt", "tsv", "spext"],
  dbFile: "",
  dbFileExtension: ["duckdb"],
  sql: "",
  sep: ",",
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
  console.log(result);
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

  const notif = $q.notify({
    color: "ongoing",
    icon: "ongoing",
    position: "bottom-right",
    group: false,
    timeout: 0,
    spinner: QSpinnerGears,
    message: "正在写入duckdb...",
  });

  const res = await invoke("connect", {
    table: data.table,
    file: data.file,
    sep: data.sep,
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

// open duckdb
async function openDBfile() {
  const selected = await open({
    multiple: false,
    filters: [
      {
        name: "duckdb",
        extensions: data.dbFileExtension,
      },
    ],
  });
  if (Array.isArray(selected)) {
    data.dbFile = selected.toString();
  } else if (selected === null) {
    Notify.create({
      type: "warning",
      message: "未选择duckdb文件",
      position: "bottom-right",
    });
    return;
  } else {
    data.dbFile = selected;
  }
}

// excute query
async function query() {
  if (data.dbFile === "") {
    Notify.create({
      type: "warning",
      message: "未选择duckdb文件",
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
    message: "正在查询...",
  });

  const res = await invoke("query", {
    file: data.dbFile,
    sep: data.sep,
    sql: data.sql,
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
      message: "查询成功,耗时: " + res,
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
