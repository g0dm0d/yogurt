import { Route, Routes } from "react-router-dom";
import { Accounts } from "./components/Accounts";
import { Home } from "./components/Home";
import { Settings } from "./components/Settings";

export default function AppRoutes() {
    return (
        <Routes>
            <Route index element={<Home />} />
            <Route path="/accounts" element={<Accounts />} />
            <Route path="/settings" element={<Settings />} />
            <Route path="*" element={<Home />} />
        </Routes>
    )
}