import {
    Box,
} from '@mantine/core';
import { InstanceCard } from './ui/instanceCard';

export function Home() {

    return (
        <Box sx={{
            display: 'flex', justifyContent: 'start', alignItems: 'start',
            height: '100%', width: '100%', padding: '80px', gap: '32px'
        }}>
            <InstanceCard title='minecraft' version='1.19.2' type='Fabric' />
            <InstanceCard title='minecraft 1.18.2' version='1.18.2' type='Fabric' />
        </Box>
    );
}