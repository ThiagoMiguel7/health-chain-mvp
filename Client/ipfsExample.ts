// ipfsExample.ts
import fs from "fs";
import path from "path";
import http from "http";

function isAsyncIterable(obj: any): obj is AsyncIterable<any> {
  return obj != null && typeof obj[Symbol.asyncIterator] === "function";
}

async function getCreateFunctionFromModule(mod: any) {
  // Tenta várias formas seguras de extrair "create" do ipfs-http-client,
  // porque o pacote pode expor de maneiras diferentes conforme a versão/empacotamento.
  if (mod == null) {
    throw new Error("Módulo ipfs-http-client importado é nulo/undefined.");
  }

  if (typeof mod.create === "function") return mod.create;
  if (typeof mod.default === "function") return mod.default;
  if (mod.default && typeof mod.default.create === "function") return mod.default.create;

  throw new Error(
    "Não foi possível extrair a função 'create' do módulo ipfs-http-client. Formatos esperados: named export 'create' ou default export."
  );
}

async function addAndGetCid(ipfs: any, fileObj: { path?: string; content: Uint8Array | Buffer | string }) {
  // O ipfs.add pode retornar:
  // - um objeto com .cid
  // - ou um AsyncIterable (dependendo da versão). Tratamos ambos.
  const maybe = ipfs.add(fileObj);

  if (isAsyncIterable(maybe)) {
    // itera e pega o último elemento (resultado final)
    let last: any = undefined;
    for await (const r of maybe) {
      last = r;
    }
    if (!last) throw new Error("ipfs.add retornou um iterable vazio.");
    return last.cid.toString();
  } else {
    // resultado direto
    // alguns retornos podem ter .cid, outros podem ser o CID diretamente
    const res = await maybe;
    if (res == null) throw new Error("ipfs.add retornou null/undefined.");
    if (res.cid) return res.cid.toString();
    // fallback (caso res seja uma string/Buffer)
    return String(res);
  }
}

function downloadFromGateway(cid: string, outFilename?: string): Promise<string> {
  return new Promise((resolve, reject) => {
    const url = `http://127.0.0.1:8080/ipfs/${cid}`;
    const outName = outFilename ?? `downloaded_${cid}`;
    const outPath = path.resolve(__dirname, outName);

    const req = http.get(url, (res) => {
      if (!res.statusCode || res.statusCode >= 400) {
        reject(new Error(`Erro HTTP ${res.statusCode} ao baixar ${url}`));
        return;
      }

      const fileStream = fs.createWriteStream(outPath);
      res.pipe(fileStream);

      fileStream.on("finish", () => {
        fileStream.close();
        resolve(outPath);
      });

      fileStream.on("error", (err) => {
        // remove arquivo parcialmente escrito
        try { fs.unlinkSync(outPath); } catch (_) {}
        reject(err);
      });
    });

    req.on("error", (err) => {
      reject(err);
    });
  });
}

async function main() {
  try {
    console.log("🔌 Importando ipfs-http-client dinamicamente...");
    const ipfsModule = await import("ipfs-http-client");
    const create = await getCreateFunctionFromModule(ipfsModule);

    // cria cliente apontando para seu Kubo local
    const ipfs = create({ url: "http://127.0.0.1:5001" });

    // arquivo hardcoded (você já criou meow.txt)
    const filePath = path.resolve(__dirname, "meow.txt");
    if (!fs.existsSync(filePath)) {
      throw new Error(`Arquivo não encontrado: ${filePath} — crie meow.txt ou ajuste o caminho.`);
    }
    const fileBuffer = fs.readFileSync(filePath);
    const fileName = path.basename(filePath);

    console.log("📤 Enviando arquivo para o IPFS (Kubo local)...");
    const cid = await addAndGetCid(ipfs, { path: fileName, content: fileBuffer });

    const gatewayUrl = `http://127.0.0.1:8080/ipfs/${cid}`;
    console.log("✅ Upload concluído!");
    console.log("CID:", cid);
    console.log("Gateway URL (local):", gatewayUrl);

    // opcional: pin (deixe comentado; descomente se quiser fixar)
    // try {
    //   await ipfs.pin.add(cid);
    //   console.log("📌 CID pinado localmente.");
    // } catch (pinErr) {
    //   console.warn("⚠️ Não foi possível pinar automaticamente:", pinErr);
    // }

    console.log("\n⬇️  Baixando via gateway local...");
    const downloadedPath = await downloadFromGateway(cid, `downloaded_${fileName}`);
    console.log("✅ Download concluído. Salvo em:", downloadedPath);

    // mostra preview se for pequeno e texto
    try {
      const stat = fs.statSync(downloadedPath);
      if (stat.size < 100 * 1024) {
        // tenta ler como utf8, se falhar (binário) ignora
        try {
          const data = fs.readFileSync(downloadedPath, "utf8");
          console.log("\n--- preview do arquivo baixado ---\n");
          console.log(data);
          console.log("\n--- fim do preview ---\n");
        } catch (_) {
          console.log("Arquivo parece binário; preview omitido.");
        }
      } else {
        console.log("Arquivo grande — preview omitido.");
      }
    } catch (_) {
      // ignore
    }

    console.log("🎉 POC finalizada.");
  } catch (err: any) {
    console.error("❌ Erro durante execução:", err && err.message ? err.message : err);
    console.error(err);
    process.exit(1);
  }
}

main();

