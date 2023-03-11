import { Flex } from '@mantine/core';
import { IconX } from '@tabler/icons-react';
import { invoke } from '@tauri-apps/api';
import { useContext } from 'react';
import { PlayerHead } from './playerHead';
import { selectedAccount } from '../../context/AccountContext';

interface AccountProps {
    nickname: string;
}

export function Account({ nickname }: AccountProps) {

    const { changeNickname } = useContext(selectedAccount);
    
    function selectAccount(nickname: string) {
        console.log(nickname);
        changeNickname(nickname);
    }

    async function deleteAccount(nickname: string) {
        try {
            await invoke('delete_account', { nickname: nickname });
        } catch (error) {
            console.error(error);
        }
    }

    return (
        <Flex direction='row' justify='space-between' sx={{ width: '100%' }} >
            <Flex gap='16px' onClick={() => selectAccount(nickname)} sx={{ cursor: 'pointer' }}>
                <PlayerHead nickname={nickname} size={30} />
                {nickname ? nickname : 'undefined'}
            </Flex>
            <IconX cursor='pointer' onClick={() => deleteAccount(nickname)} />
        </Flex>
    );
}