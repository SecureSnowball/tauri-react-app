import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import Button from '@mui/material/Button';
import Container from "@mui/material/Container";
import NavBar from "../components/NavBar";

const Home = () => {

  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("Lmao noob");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <div>
      <Button variant="contained" onClick={() => greet()}>Commit suicide</Button>
      <p>{greetMsg}</p>
    </div>
  );
}

export default Home