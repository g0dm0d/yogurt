import {
    Box, Button, Flex, Title
} from '@mantine/core';
import { Link } from 'react-router-dom';

export function Login() {

    return (
        <Box sx={{ display: 'flex', alignItems: 'center', minHeight: '30vh' }}>
            <Flex direction='column' gap='lg' justify='space-between' align='center' sx={{ textAlign: 'center' }}>
                <Title order={1}>
                    Login with your Microsoft account
                </Title>
                <Button variant='outline' target="_blank" component={Link} to='https://login.microsoftonline.com/consumers/oauth2/v2.0/authorize?client_id=d8e1d9bf-287f-4773-a176-e012722257f4&response_type=code&redirect_uri=http://localhost:9397&scope=XboxLive.signin%20offline_access&state=NOT_NEEDED'>
                    Login
                </Button>
            </Flex>
        </Box>
    );
}