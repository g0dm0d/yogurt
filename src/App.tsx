import { Box, Flex } from "@mantine/core";
import { BrowserRouter } from "react-router-dom";
import AppRoutes from "./Routes";
import { useState } from 'react';
import { Dashboard } from "./components/Dashboard";
import { selectedAccount } from "./context/AccountContext";


function App() {
    const [nickname, setNickname] = useState<string>();
    const changeNickname = (nickname: string) => {
        setNickname(nickname);
        console.log(nickname + ' changed');
    }
    return (
        <BrowserRouter>
            <selectedAccount.Provider value={{ nickname, changeNickname }} >
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
            </selectedAccount.Provider>
        </BrowserRouter >
    );
}

export default App;
