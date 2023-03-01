import { Outlet, Route, Routes } from "react-router-dom";
import { Accounts } from "./components/Accounts";
import { Settings } from "./components/Settings";

export default function AppRoutes() {
    return (
        <Routes>
            <Route index element={<Outlet />} />
            <Route path="/account" element={<Accounts />} />
            <Route path="/settings" element={<Settings />} />
            <Route path="*" element={<Outlet />} />
        </Routes>
    )
}