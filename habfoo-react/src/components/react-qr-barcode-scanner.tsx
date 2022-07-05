import React, { ChangeEvent, useState } from "react";
import Quagga from "@ericblade/quagga2";

const getFileDataUrl = async (value: File): Promise<string> => {
  return new Promise((resolve, reject) => {
    let fileReader = new FileReader();
    fileReader.readAsDataURL(value);
    fileReader.onload = () => {
      if (fileReader.result === null) {
        reject("Undefined result");
        return;
      }
      if ((fileReader.result as ArrayBuffer).byteLength !== undefined) {
        reject("Parsed array buffer somehow");
        return;
      }
      resolve(fileReader.result as string);
    };
    fileReader.onerror = (err) => reject(err);
  });
};

const parseBarcode = async (dataUrl: string): Promise<number> => {
  const result = await Quagga.decodeSingle({
    decoder: {
      readers: [
        "code_128_reader",
        "ean_reader",
        "ean_8_reader",
        "code_39_reader",
        "code_39_vin_reader",
        "codabar_reader",
        "upc_reader",
        "upc_e_reader",
        "i2of5_reader",
        "2of5_reader",
        "code_93_reader",
      ],
    },
    src: dataUrl,
  });
  console.log(result);
  const code = result?.codeResult?.code ?? undefined;
  if (code !== undefined && code !== null) {
    console.log("WTF", code);
    return parseInt(code);
  }
  throw new Error("cannot parse");
};
const onFileUpdate = async (
  event: ChangeEvent,
  setter: (barCode: number) => void
) => {
  const dataUrl = await getFileDataUrl(event.target.files[0]);
  console.log(dataUrl);
  const barCode = await parseBarcode(dataUrl);

  setter(barCode);
};

const BarcodeScannerComponent = ({
  onUpdate,
}: {
  width: number;
  height: number;
  onUpdate: (barcode?: string) => void;
}): React.ReactElement => {
  const [currentBarcode, setNewBarcode] = useState<number | undefined>(
    undefined
  );

  return (
    <>
      {currentBarcode !== undefined ? (
        <span> {currentBarcode} </span>
      ) : (
        <input
          onChange={(e) => onFileUpdate(e, setNewBarcode)}
          type="file"
          accept="image/*;capture=camera"
        />
      )}
    </>
  );
  /*
    <Webcam
      width={width}
      height={height}
      ref={webcamRef}
      onUserMediaError={(e) => {console.log(e)}}
      screenshotFormat="image/png"
      videoConstraints={{
        facingMode: 'environment'
      }}
    />
    */
};

export default BarcodeScannerComponent;
