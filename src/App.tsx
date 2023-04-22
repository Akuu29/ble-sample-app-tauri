import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

type Devices = Array<string>;

const App = () => {
  const [devices, setDevices] = useState<Devices | null>(null);

  const handleGetDevices = async () => {
    const deviceList = await invoke<Devices>("get_devices")
      .catch(err => {
        console.error(err);
        return null;
      });

    setDevices(deviceList);
  };

  const handleConnect = async (deviceName: string) => {
    // 接続
    await invoke<void>("connect_device", { deviceName })
      .catch(err => {
        console.error(err);
      });
  };

  return (
    <>
      <button onClick={handleGetDevices}>Get Device</button>
      <div>
        {devices != null && (
          <ul>
            {devices.map((device, index) => (
              <li key={index}>
                {device}
                <button onClick={() => handleConnect(device)}>Connect</button>
              </li>
            ))}
          </ul>
        )}
      </div>
    </>
  )
};

export default App;
