import { Box } from '@mantine/core';
import { useEffect, useState } from 'react';
import steveHead from '/SteveHead.png';

export function PlayerHead() {
    const [head, setHead] = useState();
    const [uuid, setUuid] = useState();

    const fetchImage = async () => {
        if (uuid) {
            const response = await fetch(`https://mc-heads.net/avatar/${uuid}/50/nohelm.png`);
            const data = await response.json();
            setHead(data.results[0].picture.large);
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