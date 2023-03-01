import { Box } from '@mantine/core';
import { useState } from 'react';
import { PlayerHead } from './playerHead';

interface AccountProps {
    nickname: string;
}

export function Account({nickname}: AccountProps)  {
    
    return (
        <Box>
            <PlayerHead nickname={nickname}/>
            {nickname}
        </Box>
    );
}