import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { Dashboard } from "./components/dashboard/Dashboard";

function App() {

    return (
        <div className="container">
            <Dashboard />
        </div>
    );
}

export default App;
