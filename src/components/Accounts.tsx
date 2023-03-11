import { useEffect, useState } from 'react';
import {
    Box,
    Button,
    Flex,
    Modal
} from '@mantine/core';
import { Login } from './popups/Login';
import { Account } from './ui/account';
import { invoke } from '@tauri-apps/api/tauri'
import { IconPlus } from '@tabler/icons-react';

async function addAccount() {
    try {
        await invoke('add_account');
    } catch (error) {
        console.error(error);
    }
}

export function Accounts() {
    const [openModal, setOpenModal] = useState(false);

    const [accounts, setAccounts] = useState<string[]>([]);
    async function getAccounts() {
        const users = await invoke<string[]>('get_all_users');
        setAccounts(users);
    }

    useEffect(() => {
        getAccounts();
    }, []);

    return (
        <Box sx={{
            display: 'flex', justifyContent: 'center',
            minHeight: '60vh', maxWidth: '40vh', width: '100%'
        }}>
            <Modal opened={openModal} onClose={() => setOpenModal(false)} title='Login'>
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
                >
                    {accounts.map((account) =>
                        <Account nickname={account} key={account} />
                    )}
                    <Button sx={{width: '100%', height: 40}} variant='outline' onClick={() => setOpenModal(true)}>
                        <IconPlus />
                    </Button>
                </Flex>
            </Flex>
        </Box >
    );
}