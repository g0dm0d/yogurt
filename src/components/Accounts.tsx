/* eslint-disable @typescript-eslint/explicit-function-return-type */
/* eslint-disable react/react-in-jsx-scope */
import { useContext, useEffect, useState } from 'react'
import {
  Box,
  Button,
  Flex,
  Modal
} from '@mantine/core'
import { Login } from './popups/Login'
import { Account } from './ui/account'
import { invoke } from '@tauri-apps/api/tauri'
import { IconPlus } from '@tabler/icons-react'
import { selectedAccount } from '../context/AccountContext'
import { useEventListener } from '@mantine/hooks'

export function Accounts () {
  const [openModal, setOpenModal] = useState(false)
  const { nickname, changeNickname } = useContext(selectedAccount)
  const [accounts, setAccounts] = useState<string[]>([])
  async function getAccounts () {
    const accounts = await invoke<string[]>('get_all_users')
    setAccounts(accounts)
    if (nickname == null) {
      changeNickname?.(accounts[0])
    }
  }

  const ref = useEventListener('click', getAccounts)

  useEffect(() => {
    void getAccounts()
  }, [ref])

  return (
        <Box sx={{
          display: 'flex',
          justifyContent: 'center',
          minHeight: '60vh',
          maxWidth: '40vh',
          width: '100%'
        }}>
            <Modal opened={openModal} onClose={() => { setOpenModal(false) }} title='Login'>
                <Login />
            </Modal>

            <Flex
                justify='space-between'
                align='flex-start'
                direction='column'
                sx={{ width: '100%', padding: '16px' }}
            >
                <Flex
                    gap='md'
                    direction='column'
                    sx={{ width: '100%' }}
                    ref={ref}
                >
                    {accounts.map((account) =>
                        <Account nickname={account} key={account} />
                    )}
                    <Button sx={{ width: '100%', height: 40 }} variant='outline' onClick={() => { setOpenModal(true) }}>
                        <IconPlus />
                    </Button>
                </Flex>
            </Flex>
        </Box >
  )
}
