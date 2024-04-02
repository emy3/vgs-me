import React, { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

const App: React.FC = () => {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    try {
      const response = await invoke("greet", { name });
      setGreetMsg(response as string);
    } catch (error) {
      console.error("Error:", error);
    }
  }

  return (
    <div className="container">
      <h1>Welcome to VGS Me</h1>

      {/* TODO:
          - refactor to pick random text from JSON
      */}
      <p>You rock!</p>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Submit</button>
      </form>

      <p>{greetMsg}</p>
    </div>
  );
}

export default App;
