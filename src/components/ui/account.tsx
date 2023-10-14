/* eslint-disable @typescript-eslint/no-misused-promises */
import { Flex } from '@mantine/core'
import { IconX } from '@tabler/icons-react'
import { invoke } from '@tauri-apps/api'
import { useContext } from 'react'
import { PlayerHead } from './playerHead'
import { selectedAccount } from '../../context/AccountContext'

interface AccountProps {
  nickname: string
}

export function Account ({ nickname }: AccountProps): JSX.Element {
  const { changeNickname } = useContext(selectedAccount)

  function selectAccount (nickname: string): void {
    changeNickname?.(nickname)
  }

  async function deleteAccount (nickname: string): Promise<void> {
    try {
      await invoke('delete_account', { name: nickname })
    } catch (error) {
      console.error(error)
    }
  }

  return (
        <Flex direction='row' justify='space-between' sx={{ width: '100%' }} >
            <Flex gap='16px' onClick={() => { selectAccount(nickname) }} sx={{ cursor: 'pointer' }}>
                <PlayerHead nickname={nickname} size={30} />
                {nickname ?? 'undefined'}
            </Flex>
            <IconX cursor='pointer' onClick={async () => { await deleteAccount(nickname) }} />
        </Flex>
  )
}
