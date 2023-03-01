import { Box, Flex } from '@mantine/core';
import { useState } from 'react';
import { PlayerHead } from './playerHead';

interface AccountProps {
    nickname?: string;
}

export function Account({ nickname }: AccountProps) {

    return (
        <Flex direction='row' gap='16px' >
            <PlayerHead nickname={nickname} size={30} />
            {nickname ? nickname : 'undefined'}
        </Flex>
    );
}