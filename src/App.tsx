import { Box, Flex } from "@mantine/core";
import { BrowserRouter } from "react-router-dom";
import AppRoutes from "./Routes";
import { useState, useEffect } from 'react';
import { Dashboard } from "./components/Dashboard";
import { selectedAccount } from "./context/AccountContext";
import { invoke } from "@tauri-apps/api";
import Welcome from "./components/Welcome";


function App() {
  const [accounts, setAccounts] = useState<string[]>([]);
  const [nickname, setNickname] = useState<string>();
  const changeNickname = (nickname: string) => {
    setNickname(nickname);
  }

  async function getAccounts() {
    const allUsers = await invoke<string[]>('get_all_users');
    console.log(allUsers);
    setAccounts(allUsers);
    if (!allUsers.length) {
      localStorage.removeItem('selectedAccount');
      setNickname(undefined);
    } else {
      if (!nickname) {
        const localNickname = localStorage.getItem('selectedAccount')
        if (localNickname) {
          setNickname(localNickname);
        } else if (allUsers.length) {
          setNickname(allUsers[0]);
          localStorage.setItem('selectedAccount', allUsers[0]);
        }
      }
    }
  }

  useEffect(() => {
    getAccounts();
  }, [])

  const onAddAccount = () => {
    getAccounts();
  }

  if (nickname) {
    return (
      <BrowserRouter>
        <selectedAccount.Provider value={{ nickname, changeNickname }} >
          <Flex direction="row">
            <Dashboard />
            <Box sx={(theme) => ({
              display: 'flex', width: '100%', height: '100vh',
              backgroundColor: theme.colors.dark[5], justifyContent: 'center', alignItems: 'center'
            })}>
              <AppRoutes />
            </Box>
          </Flex>
        </selectedAccount.Provider>
      </BrowserRouter >
    );
  } else {
    return (
      <Welcome onAddAccount={onAddAccount} />
    );
  }
}

export default App;
