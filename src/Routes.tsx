import { Outlet, Route, Routes } from "react-router-dom";
import { Account } from "./components/Account";
import { Settings } from "./components/Settings";

export default function AppRoutes() {
    return (
        <Routes>
            <Route index element={<Outlet />} />
            <Route path="/account" element={<Account />} />
            <Route path="/settings" element={<Settings />} />
            <Route path="*" element={<Outlet />} />
        </Routes>
    )
}