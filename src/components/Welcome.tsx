/* eslint-disable @typescript-eslint/no-misused-promises */
/* eslint-disable react/react-in-jsx-scope */
/* eslint-disable @typescript-eslint/explicit-function-return-type */
import {
  Box, Button, Flex, Title
} from '@mantine/core'
import { invoke } from '@tauri-apps/api/tauri'
import { type FC } from 'react'

const MICROSOFT_LINK = 'https://login.microsoftonline.com/consumers/oauth2/v2.0/authorize?client_id=d8e1d9bf-287f-4773-a176-e012722257f4&response_type=code&redirect_uri=http://localhost:9397&scope=XboxLive.signin%20offline_access&state=NOT_NEEDED'

interface WelcomePageProps {
  onAddAccount: () => void
}

const Welcome: FC<WelcomePageProps> = ({ onAddAccount }) => {
  async function addAccount () {
  // open(MICROSOFT_LINK);
    await invoke('add_account')
      .then((response) => {
        onAddAccount()
      })
      .catch((error) => { console.error(error) })
  }

  return (
      <Box sx={{ display: 'flex', alignItems: 'center', justifyContent: 'center', width: '100wh', height: '100vh' }} >
          <Flex direction='column' gap={32} sx={{ display: 'flex', textAlign: 'center', justifyContent: 'center', alignContent: 'center', width: 400 }}>
              <Title order={1} sx={(theme) => ({ color: theme.primaryColor })}>
                  Welcome to yogurt Minecraft Launcher!
              </Title>
              <Title order={3}>
                  Login with your Microsoft account to continue
              </Title>
              <Button onClick={addAccount} target='_blank' component='a' href={MICROSOFT_LINK}>
                  Login
              </Button>
          </Flex>
      </Box>
  )
}

export default Welcome
