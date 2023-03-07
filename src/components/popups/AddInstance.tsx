import {
    Box,
    Button,
    Flex,
    Select,
} from '@mantine/core';
import { IconChevronDown } from '@tabler/icons-react';
import { invoke } from '@tauri-apps/api/tauri';
import { useState, useEffect } from 'react';

type Version = {
    label: string;
    value: string;
};

async function createInstance(label: string, value: string) {
    if (label && value) {
        try {
            await invoke('get_minecraft', {
                url: value,
                id: label,
                name: '',
                javaArgs: ''
            });
            console.log(Response);
        } catch (error) {
            console.error(error);
        }
    }
}

export function AddInstance() {

    const [versions, setVersions] = useState<Version[]>([]);
    const [value, setValue] = useState<string>('');
    const label = versions.find((item) => item.value === value)?.label

    const [loading, setLoading] = useState(false);
    async function getVersions() {
        setLoading(true);
        fetch('https://launchermeta.mojang.com/mc/game/version_manifest_v2.json')
            .then(response => response.json())
            .then(
                (result) => {
                    setLoading(false);
                    for (let i = 0; i < result.versions.length; i++) {
                        const versionObj: Version = { label: result.versions[i].id, value: result.versions[i].url };
                        versions.push(versionObj);
                        // setVersions([...versions, versionObj]);
                    }
                },
                (error) => {
                    setLoading(false);
                    console.error(error);
                    setValue('error');
                }
            )
    }

    useEffect(() => {
        getVersions();
    }, []);


    return (
        <Box sx={{ display: 'flex', alignItems: 'center', minHeight: '30vh', height: '100%', width: '100%' }}>
            <Flex direction='column' gap='lg' justify='space-between' sx={{ height: '100%', width: '100%' }}>
                <Select
                    data={versions}
                    value={value}
                    onChange={setValue}
                    color='white'
                    description="Version"
                    variant="filled"
                    size="md"
                    placeholder="Version"
                    searchable
                    nothingFound="Error"
                    rightSection={<IconChevronDown size="1rem" />}
                    transition='fade'
                    transitionDuration={200}
                />
                <Button onClick={() => createInstance(label, value)} variant='outline' >
                    Create
                </Button>
            </Flex>
        </Box>
    );
}