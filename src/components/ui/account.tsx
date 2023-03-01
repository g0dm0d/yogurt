import { Flex } from '@mantine/core';
import { IconX } from '@tabler/icons-react';
import { PlayerHead } from './playerHead';

interface AccountProps {
    nickname?: string;
}

export function Account({ nickname }: AccountProps) {

    return (
        <Flex direction='row' justify='space-between' sx={{ width: '100%' }}>
            <Flex gap='16px'>
                <PlayerHead nickname={nickname} size={30} />
                {nickname ? nickname : 'undefined'}
            </Flex>
            <IconX cursor='pointer'/>
            {/* onClick={deleteAccount(nickname)} */}
        </Flex>
    );
}