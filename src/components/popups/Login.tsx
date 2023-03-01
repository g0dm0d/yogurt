import { useState } from 'react';
import {
    Box
} from '@mantine/core';
import { Link } from 'react-router-dom';

export function Login() {

    return (
        <Box sx={(theme) => ({ display: 'flex', justifyContent: 'center', alignItems: 'center', minHeight: '60vh', width: '100%' })}>
            Login Form
        </Box>
    );
}