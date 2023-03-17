import {
    Box,
    Button,
    Flex,
    Title
} from '@mantine/core';
import { invoke } from '@tauri-apps/api/tauri';
import { SettingsDashboard } from './SettingsDashboard';

export function InstanceSettings() {

    return (
        <Box sx={{ display: 'flex', alignItems: 'center', minHeight: '30vh', width: '80wh' }}>
            <SettingsDashboard />
        </Box>
    );
}
