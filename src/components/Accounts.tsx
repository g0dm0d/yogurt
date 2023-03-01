import { useMemo, useState } from 'react';
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
    const [openPopup, setOpenPopup] = useState(false);
    const closePopup = () => setOpenPopup(false);

    const [accounts, setAccounts] = useState(['ModerNik', 'STN0WHERE']);

    return (
        <Box sx={{
            display: 'flex', justifyContent: 'center',
            minHeight: '60vh', maxWidth: '40vh', width: '100%'
        }}>
            <Modal opened={openPopup} onClose={closePopup} title='Login'>
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
                    <Account nickname={accounts[0]} />
                    <Account nickname={accounts[1]} />
                </Flex>
                <Button variant='outline' onClick={() => setOpenPopup(true)}>+</Button>
            </Flex>
        </Box >
    );
}