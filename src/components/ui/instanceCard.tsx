import {
    createStyles,
    Card,
    Image,
    Text,
    Box,
    Menu,
} from '@mantine/core';
import { useHover } from '@mantine/hooks';
import {
    IconAdjustmentsHorizontal,
    IconCopy,
    IconFolder,
    IconPlayerPlay,
    IconSettings,
    IconTrash
} from '@tabler/icons-react';
import { invoke } from '@tauri-apps/api';
import { useContext, useState } from 'react';
import { selectedAccount } from '../../context/AccountContext';
import bg from '/bg.png';

interface InstanceCardProps {
    name: string;
    version: string;
    gameType: string;
}

async function startInstance(name: string, username: string | undefined) {
    if (name) {
        try {
            await invoke('run_minecraft', {
                username: username,
                instance: name
            });
        } catch (error) {
            console.error(error);
        }
    }
}

async function deleteInstance(name: string) {
    if (name) {
        try {
            console.log(name);
            await invoke('delete_instance', {
                name
            });
        } catch (error) {
            console.error(error);
        }
    }
}

async function openFolder(name: string) {
    if (name) {
        try {
            await invoke('open_instance_folder', {
                name: name
            });
        } catch (error) {
            console.error(error);
        }
    }
}

async function createInstance(name: string, version: string | undefined, type: string, url?: string | null) {
    if (!url) {
        fetch('https://launchermeta.mojang.com/mc/game/version_manifest_v2.json')
            .then(response => response.json())
            .then(
                (result) => {
                    for (let i = 0; i < result.versions.length; i++) {
                        if (result.versions[i].id === version) {
                            url = result.versions[i].url;
                        }
                    }
                    name = name + ' copy'
                },
                (error) => {
                    console.error(error);
                }
            )
            .then(() => {
                getMinecraft();
            })
    } else {
        getMinecraft();
    }

    async function getMinecraft() {
        await invoke('get_minecraft', {
            url: url,
            id: version,
            name: name,
            javaArgs: '-Xmx4G',
            fabric: type === 'fabric' ? true : false
        })
            .then((response) => {
                console.log(response);
            })
            .catch((error) => console.error(error))
    }
}

export function InstanceCard({ name, version, gameType }: InstanceCardProps) {
    const { hovered, ref } = useHover();
    const useStyles = createStyles((theme) => ({
        card: {
            width: '216px',
            height: '164px',
            backgroundColor: theme.colorScheme === 'dark' ? theme.colors.dark[7] : theme.white,
        },

        footer: {
            display: 'flex',
            justifyContent: 'space-between',
            alignItems: 'center',
            padding: `${theme.spacing.sm}px ${theme.spacing.lg}px`,
        },

        image: {
            transition: '300ms',
            filter: hovered ? 'brightness(0.7) blur(2px)' : 'brightness(1) blur(2px)',
        },

        playIcon: {
            transition: '300ms',
            position: 'absolute',
            opacity: hovered ? 1 : 0,
        },

        name: {
            color: theme.colorScheme === 'dark' ? theme.colors.dark[1] : theme.colors.gray[1],
            fontSize: theme.fontSizes.lg,
            transition: '300ms',
            position: 'absolute',
            opacity: hovered ? 0 : 1,
        },
    }));
    const { classes } = useStyles();
    const [openMenu, setOpenMenu] = useState(false);
    const { nickname } = useContext(selectedAccount);
    return (
        <Menu opened={openMenu} onChange={setOpenMenu} withArrow>
            <Card p="lg" className={classes.card}>
                <Card.Section>
                    <Box ref={ref} display='flex' onClick={() => startInstance(name, nickname)}
                        sx={{
                            justifyContent: 'center',
                            alignItems: 'center',
                            cursor: 'pointer'
                        }} >
                        <Image className={classes.image} src={bg} alt={name} height={100} />
                        <Text size="sm" weight={700} className={classes.name}>
                            {name}
                        </Text>
                        <IconPlayerPlay className={classes.playIcon} />
                    </Box>
                </Card.Section>
                <Card.Section className={classes.footer}>
                    <Box>
                        <Text size="xs" color="dimmed">
                            Version
                        </Text>
                        <Text weight={500} size="sm">
                            {version}
                        </Text>
                    </Box>
                    <Box>
                        <Text size="xs" color="dimmed">
                            Type
                        </Text>
                        <Text weight={500} size="sm">
                            {gameType}
                        </Text>
                    </Box>
                    <Box>
                        <Menu.Target>
                            <IconAdjustmentsHorizontal cursor='pointer' onClick={() => setOpenMenu(true)} />
                        </Menu.Target>
                    </Box>
                </Card.Section>
            </Card>
            <Menu.Dropdown >
                <Menu.Item icon={<IconSettings size={14} />}>
                    Settings
                </Menu.Item>
                <Menu.Item icon={<IconFolder size={14} />} onClick={() => openFolder(name)} >
                    Folder
                </Menu.Item>
                <Menu.Item icon={<IconCopy size={14} />} onClick={() => createInstance(name, version, gameType)}>
                    Make copy
                </Menu.Item>
                <Menu.Divider />

                <Menu.Item color="red" icon={<IconTrash size={14} />} onClick={() => deleteInstance(name)}>
                    Delete instance
                </Menu.Item>
            </Menu.Dropdown>
        </Menu>
    );
}