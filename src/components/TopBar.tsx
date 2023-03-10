import { useState } from 'react';
import {
    Box
} from '@mantine/core';
import { Link } from 'react-router-dom';
import { IconBrowser } from '@tabler/icons-react';


export function TopBar() {

    return (
        <Box sx={(theme) => ({
            display: 'flex', height: '30px', paddingRight: '8px', gap: '8px',
            justifyContent: 'end', alignItems: 'center',
            backgroundColor: theme.colors.dark[7]
        })}>
            <IconBrowser color='white' stroke={1.5} />
            <IconBrowser color='white' stroke={1.5} />
            <IconBrowser color='white' stroke={1.5} />
        </Box>
    );
}