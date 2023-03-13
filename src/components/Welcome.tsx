import {
    Box, Button, Flex, Title
} from '@mantine/core';
import { invoke } from '@tauri-apps/api/tauri';
import { open } from '@tauri-apps/api/shell';

const MICROSOFT_LINK = 'https://login.microsoftonline.com/consumers/oauth2/v2.0/authorize?client_id=d8e1d9bf-287f-4773-a176-e012722257f4&response_type=code&redirect_uri=http://localhost:9397&scope=XboxLive.signin%20offline_access&state=NOT_NEEDED'

async function addAccount() {
    open(MICROSOFT_LINK)
    try {
        await invoke('add_account');
        console.log(Response);
    } catch (error) {
        console.error(error);
    }
}

export function Welcome() {

    return (
        <Box sx={{ display: 'flex', alignItems: 'center', justifyContent: 'center' }}>
            <Title order={1} sx={(theme) => ({ color: theme.primaryColor })}>
                Welcome to yogurt Minecraft Launcher!
            </Title>
            <Title  order={3}>
                Login with your Microsoft account to continue
            </Title>
            <Button onClick={addAccount} >
                Login
            </Button>
        </Box>
    );
}