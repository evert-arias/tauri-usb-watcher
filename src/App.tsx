import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import "./App.css";

function App() {
  const [notifications, setNotifications] = useState<
    { id: number; message: string; timestamp: string }[]
  >([]);

  useEffect(() => {
    const unlisten = listen("usb-event", (event) => {
      setNotifications((prev) => [
        ...prev,
        {
          id: Date.now(),
          message: event.payload as string,
          timestamp: new Date().toLocaleTimeString(),
        },
      ]);
    });

    return () => {
      unlisten.then((f) => f());
    };
  }, []);

  return (
    <div className="container">
      <h1>USB Device Monitor</h1>
      <div className="notifications">
        {notifications.map((notification) => (
          <div key={notification.id} className="notification">
            <span className="time">{notification.timestamp}</span>
            <span className="message">{notification.message}</span>
          </div>
        ))}
      </div>
    </div>
  );
}

export default App;
