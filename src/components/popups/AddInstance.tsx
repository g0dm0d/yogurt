import {
    Box,
    Button,
    Flex,
    Select,
} from '@mantine/core';
import { IconChevronDown } from '@tabler/icons-react';
import { invoke } from '@tauri-apps/api/tauri';
import { useState, useEffect } from 'react';

async function addInstance() {
    try {
        await invoke('add_instance');
        console.log(Response);
    } catch (error) {
        console.error(error);
    }
}

type Version = {
    label: string;
    value: string;
};

export function AddInstance() {

    const [versions, setVersions] = useState<Version[]>([]);
    async function getVersions() {
        const response = await fetch('https://launchermeta.mojang.com/mc/game/version_manifest_v2.json')
        const data = await response.json();
        for (let i = 0; i < data.versions.length; i++) {
            const versionObj: Version = { label: data.versions[i].id, value: data.versions[i].url };
            versions.push(versionObj);
            // setVersions([...versions, versionObj]);
        }
    }

    useEffect(() => {
        getVersions();
    }, []);
    return (
        <Box sx={{ display: 'flex', alignItems: 'center', minHeight: '30vh', height: '100%', width: '100%' }}>
            <Flex direction='column' gap='lg' justify='space-between' sx={{ height: '100%', width: '100%' }}>
                <Select
                    data={versions}
                    color='white'
                    description="Version"
                    variant="filled"
                    size="md"
                    placeholder="Version"
                    searchable
                    nothingFound="No such version"
                    rightSection={<IconChevronDown size="1rem" />}
                    transition='fade'
                    transitionDuration={200}
                />
                <Button onClick={addInstance} variant='outline' >
                    Create
                </Button>
            </Flex>
        </Box>
    );
}