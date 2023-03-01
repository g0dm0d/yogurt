import { Box } from '@mantine/core';
import { useEffect, useState } from 'react';
import steveHead from '/SteveHead.png';

interface PlayerHeadProps {
    nickname?: string;
}

export function PlayerHead({nickname}: PlayerHeadProps) {
    const [head, setHead] = useState();
    const [uuid, setUuid] = useState(nickname ? nickname : undefined);

    const fetchImage = async () => {
        if (uuid && !head) {
            const response = await fetch(`https://mc-heads.net/avatar/${uuid}/50`);
            const data = await response.blob();
            const imageHead = URL.createObjectURL(data);
            console.log(imageHead);
            setHead(imageHead);
        }
    };

    useEffect(() => {
        if (!head) {
            fetchImage();
        }
    }, []);

    return (
        <Box>
            <img src={head ? head : steveHead} width='50' height='50' />
        </Box>
    );
}

