import { Box, Container, Flex } from "@mantine/core";
import { BrowserRouter } from "react-router-dom";
import { Dashboard } from "./components/Dashboard";
import { TopBar } from "./components/TopBar";
import AppRoutes from "./Routes";

function App() {

    return (
        <BrowserRouter>
            <Flex direction="row">
                <Dashboard />
                <Box sx={(theme) => ({
                    display: 'flex', width: '100%', height: '100vh',
                    backgroundColor: theme.colors.dark[5], justifyContent: 'center', alignItems: 'center'
                })}>
                    {/* <TopBar /> */}
                    <AppRoutes />
                </Box>
            </Flex>
        </BrowserRouter>
    );
}

export default App;
