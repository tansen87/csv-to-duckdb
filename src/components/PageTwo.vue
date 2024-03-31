<template>
  <div
    class="q-pa-md"
    style="display: flex; justify-content: space-between; max-width: 1000px; margin: 0 auto"
  >
    <!-- column 1 -->
    <div class="q-gutter-y-md" style="flex-basis: 46%; max-width: 300px">
      <q-input color="teal" outlined v-model="input.replCols" label="Columns-replace char">
        <template v-slot:prepend>
          <q-icon name="cloud" />
        </template>
      </q-input>
      <q-input color="teal" outlined v-model="input.cn2pinyinCols" label="Columns-CN2Pinyin">
        <template v-slot:prepend>
          <q-icon name="cloud" />
        </template>
      </q-input>
      <q-input
        color="teal"
        outlined
        v-model="input.replSfCols"
        label="Columns-replace specific char"
      >
        <template v-slot:prepend>
          <q-icon name="cloud" />
        </template>
      </q-input>
    </div>

    <!-- column 2 -->
    <div class="q-gutter-y-md" style="flex-basis: 46%; max-width: 300px">
      <q-input color="teal"></q-input>
      <q-input color="teal">
        <template v-slot:prepend>
          <q-select
            v-model="input.sep"
            :options="sepOptions"
            label="Separator"
            style="width: 145px"
          />
          <q-select
            v-model="input.encoding"
            :options="encodingOptions"
            label="Encoding"
            style="width: 145px"
          />
        </template>
      </q-input>
      <q-input color="teal" label="old char">
        <template v-slot:prepend>
          <q-icon name="arrow_right" />
          <q-input color="primary" label="old char" v-model="input.oldChar" style="width: 135px" />
          <q-icon name="arrow_right" />
          <q-input
            color="secondary"
            label="new char"
            v-model="input.newChar"
            style="width: 135px"
          />
        </template>
      </q-input>
    </div>

    <!-- column 3 -->
    <div class="q-gutter-y-md" style="flex-basis: 46%; max-width: 300px">
      <q-input color="teal">
        <template v-slot:prepend>
          <q-btn color="primary" label="open file" @click="openFile" style="width: 145px" />
          <q-btn color="secondary" label="replace" @click="replChar" style="width: 145px" />
        </template>
      </q-input>
      <q-input color="teal">
        <template v-slot:prepend>
          <q-btn color="primary" label="open file" @click="openFile" style="width: 145px" />
          <q-btn color="secondary" label="CN2Pinyin" @click="cn2pinyin" style="width: 145px" />
        </template>
      </q-input>
      <q-input color="teal">
        <template v-slot:prepend>
          <q-btn color="primary" label="open file" @click="openFile" style="width: 145px" />
          <q-btn color="secondary" label="replace-sp" @click="replSfChar" style="width: 145px" />
        </template>
      </q-input>
    </div>
  </div>

  <div
    class="q-pa-md"
    style="display: flex; justify-content: space-between; max-width: 1000px; margin: 0 auto"
  >
    <q-table
      :rows="tableData"
      :columns="columns"
      row-key="name"
      binary-state-sort
      style="height: 355px; width: 100%"
    >
    </q-table>
  </div>
</template>

<script setup>
import { ref, reactive } from 'vue'
import { useQuasar } from 'quasar'

const $q = useQuasar()

const tableData = ref([])
const columns = ref([])
const input = reactive({
  cn2pinyinCols: 'A|B|C',
  oldChar: '',
  newChar: '',
  replCols: 'A|B|C',
  replSfCols: 'A|B|C',
  sep: ',',
  encoding: 'utf-8'
})
const sepOptions = [',', '|', '\\t', ';']
const encodingOptions = ['utf-8', 'utf_8_sig', 'utf-16le', 'gbk']
</script>
