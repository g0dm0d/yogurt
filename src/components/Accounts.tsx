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

async function addAccount() {
    try {
        await invoke('add_account');
        console.log(Response);
    } catch (error) {
        console.error(error);
    }
}

export function Accounts() {
    const [openModal, setOpenModal] = useState(false);

    const [accounts, setAccounts] = useState([]);
    async function getAccounts() {
        const users = await invoke('get_all_users');
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
                </Flex>
                <Button variant='outline' onClick={() => setOpenModal(true)}>+</Button>
            </Flex>
        </Box >
    );
}