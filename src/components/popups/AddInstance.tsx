import {
    Box,
    Button,
    Center,
    Flex,
    SegmentedControl,
    Select,
    Image,
    TextInput,
} from '@mantine/core';
import { IconChevronDown } from '@tabler/icons-react';
import { invoke } from '@tauri-apps/api/tauri';
import { useState, useEffect } from 'react';
import FabricIcon from '../ui/icons/FabricIcon.svg'
import MinecraftIcon from '../ui/icons/MinecraftIcon.svg'

type Version = {
    label: string;
    value: string;
};

export async function createInstance(name: string, version: string, type: string, url?: string) {
    console.log('Creating instance:', name, version, type, url)
    if (type === 'fabric') {

    }
    // try {
    //     await invoke('get_minecraft', {
    //         url: url,
    //         id: version,
    //         name: name,
    //         javaArgs: '-Xmx4G'
    //     });
    //     console.log(Response);
    // } catch (error) {
    //     console.error(error);
    // }
}

export function AddInstance() {

    const [type, setType] = useState('minecraft');
    const [name, setName] = useState('');

    const [versions, setVersions] = useState<Version[]>([]);
    const [fabricVersions, setFabricVersions] = useState<Version[]>([]);

    const [value, setValue] = useState<string>('');
    const label = versions.find((item) => item.value === value)?.label

    const [loading, setLoading] = useState(false);
    async function getDefaultVersions() {
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
                    getFabcricVersions();
                },
                (error) => {
                    setLoading(false);
                    console.error(error);
                    setValue('error');
                }
            )
    }

    async function getFabcricVersions() {
        setLoading(true);
        fetch('https://meta.fabricmc.net/v2/versions/game')
            .then(response => response.json())
            .then(
                (result) => {
                    setLoading(false);
                    for (let i = 0; i < result.length; i++) {
                        const versionObj: Version = { label: result[i].version, value: versions.find(result[i].version) };
                        fabricVersions.push(versionObj);
                        // setFabricVersions([...fabricVersions, versionObj]);
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
        getDefaultVersions();
    }, []);

    return (
        <form onSubmit={(e) => {
            e.preventDefault();
            createInstance(name, label, type, value);
        }}>
            <Box sx={{ display: 'flex', alignItems: 'center', minHeight: '30vh', height: '100%', width: '100%' }}>
                <Flex direction='column' gap='lg' justify='space-between' sx={{ height: '100%', width: '100%' }}>
                    <TextInput
                        placeholder="Name"
                        label="Instance name"
                        value={name}
                        onChange={(event) => setName(event.currentTarget.value)}
                        required
                    />
                    <SegmentedControl
                        value={type}
                        onChange={(value: 'minecraft' | 'fabric') => setType(value)}
                        data={[
                            {
                                value: 'minecraft',
                                label: (
                                    <Center>
                                        <img width={24} height={24} src={MinecraftIcon} alt="Minecraft Icon" />
                                        <Box ml={10}>Minecraft</Box>
                                    </Center>
                                ),
                            },
                            {
                                value: 'fabric',
                                label: (
                                    <Center>
                                        <Image width={24} height={24} src={FabricIcon} alt="Fabric Icon" />
                                        <Box ml={10}>Fabric</Box>
                                    </Center>
                                ),
                            },
                        ]}
                    />
                    <Select
                        required
                        data={type === 'minecraft' ? versions : fabricVersions}
                        value={value}
                        onChange={setValue}
                        description="Version"
                        placeholder='Version'
                        searchable
                        nothingFound="Error"
                        rightSection={<IconChevronDown size="1rem" />}
                        transition='fade'
                        transitionDuration={200}
                    />
                    <Button type='submit' variant='outline' >
                        Create
                    </Button>
                </Flex>
            </Box >
        </form >
    );
}