import { BaseDirectory, readTextFile, writeTextFile } from "@tauri-apps/plugin-fs"

export async function read<T>(file: string): Promise<T> {
  try {
    const json = await readTextFile(file, {
      baseDir: BaseDirectory.AppConfig
    });
    console.log(json)
    return JSON.parse(json);
  } catch (err) {
    console.error('Erro ao ler o arquivo:', err);
    throw err;
  }
}

export async function write<T>(filename: string, data: T): Promise<void> {
  try {
    await writeTextFile(filename, JSON.stringify(data, null, 2), {
      baseDir: BaseDirectory.AppConfig,
      create: true,
      createNew: true,
    });
  } catch (err) {
    console.error('Erro ao escrever o arquivo:', err);
    throw err;
  }
}
