import { Box, Flex } from '@mantine/core'
import { BrowserRouter } from 'react-router-dom'
import AppRoutes from './Routes'
import { useState, useEffect } from 'react'
import { Dashboard } from './components/Dashboard'
import { selectedAccount } from './context/AccountContext'
import { invoke } from '@tauri-apps/api'
import Welcome from './components/Welcome'

function App (): JSX.Element {
  const [, setAccounts] = useState<string[]>([])
  const [nickname, setNickname] = useState<string>()
  const changeNickname = (nickname: string): void => {
    setNickname(nickname)
  }

  async function getAccounts (): Promise<void> {
    const allUsers = await invoke<string[]>('get_all_users')
    console.log(allUsers)
    setAccounts(allUsers)
    if (allUsers.length === 0) {
      localStorage.removeItem('selectedAccount')
      setNickname(undefined)
    } else {
      if (nickname == null) {
        const localNickname = localStorage.getItem('selectedAccount')
        if (localNickname != null) {
          setNickname(localNickname)
        } else if (allUsers.length > 0) {
          setNickname(allUsers[0])
          localStorage.setItem('selectedAccount', allUsers[0])
        }
      }
    }
  }

  useEffect(() => {
    void getAccounts()
  }, [])

  const onAddAccount = (): void => {
    void getAccounts()
  }

  if (nickname != null) {
    return (
      <BrowserRouter>
        <selectedAccount.Provider value={{ nickname, changeNickname }} >
          <Flex direction="row">
            <Dashboard />
            <Box sx={(theme) => ({
              display: 'flex',
              width: '100%',
              height: '100vh',
              backgroundColor: theme.colors.dark[5],
              justifyContent: 'center',
              alignItems: 'center'
            })}>
              <AppRoutes />
            </Box>
          </Flex>
        </selectedAccount.Provider>
      </BrowserRouter >
    )
  } else {
    return (
      <Welcome onAddAccount={onAddAccount} />
    )
  }
}

export default App
