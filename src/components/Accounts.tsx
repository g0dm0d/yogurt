import { useMemo, useState } from 'react';
import {
    Box,
    Button,
    Flex,
    Modal
} from '@mantine/core';
import { Login } from './popups/Login';
import { Account } from './ui/account';

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
                sx={{ width: '100%' }}
            >
                <Flex
                    gap='md'
                    direction='column'
                    wrap='wrap'
                >
                    <Account nickname={accounts[0]}/>
                    <Account nickname={accounts[1]}/>
                </Flex>
                <Button onClick={() => setOpenPopup(true)}>+</Button>
            </Flex>
        </Box >
    );
}