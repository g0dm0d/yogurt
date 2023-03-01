import { useState } from 'react';
import {
    Box, Button, Flex, Modal, Transition
} from '@mantine/core';
import { Link } from 'react-router-dom';
import { Login } from './popups/Login';
export function Account() {
    const [openPopup, setOpenPopup] = useState(false);
    const closePopup = () => setOpenPopup(false);

    const overlayStyle = {
        background: 'rgba(0,0,0,0.5)',
    };

    return (
        <Box sx={{
            display: 'flex', justifyContent: 'center', alignItems: 'start',
            minHeight: '60vh', maxWidth: '40vh', width: '100%', paddingTop: '10%', backgroundColor: 'grey'
        }}>
            <Modal opened={openPopup} onClose={closePopup} title='Login'>
                <Login />
            </Modal>


            <Box sx={{
                display: 'flex', justifyContent: 'center', alignItems: 'center', direction: 'column',
            }} >
                <Box sx={{}} >
                    accounts
                </Box>
                <Button onClick={() => setOpenPopup(true)}>+</Button>
            </Box>
        </Box>
    );
}