import { Container, Flex } from "@mantine/core";
import { BrowserRouter } from "react-router-dom";
import { Dashboard } from "./components/Dashboard";
import AppRoutes from "./Routes";

function App() {

    return (
        <BrowserRouter>
            <Flex direction="row">
                <Dashboard />
                <AppRoutes />
            </Flex>
        </BrowserRouter>
    );
}

export default App;
