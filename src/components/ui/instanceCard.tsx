import {
    createStyles,
    Card,
    Image,
    Text,
    Box,
    Menu,
} from '@mantine/core';
import { useHover } from '@mantine/hooks';
import { IconAdjustmentsHorizontal, IconPlayerPlay, IconSettings, IconTrash } from '@tabler/icons-react';
import { invoke } from '@tauri-apps/api';
import { useState } from 'react';
import bg from '/bg.png';

interface InstanceCardProps {
    name: string;
    version: string;
    type: string;
}

async function startInstance(instance: string, username: string) {
    if (instance) {
        try {
            await invoke('run_minecraft', {
                username: username,
                instance: instance
            });
            console.log(Response);
        } catch (error) {
            console.error(error);
        }
    }
}

export function InstanceCard({ name, version, type }: InstanceCardProps) {
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
    const [openMenu, setOpenMenu] = useState(false)
    const [username, setUsername] = useState('');
    return (
        <Menu opened={openMenu} onChange={setOpenMenu} withArrow>
            <Card p="lg" className={classes.card}>
                <Card.Section>
                    <Box ref={ref} display='flex' onClick={() => startInstance(name, username)}
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
                            {type}
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
                <Menu.Label>Application</Menu.Label>
                <Menu.Item icon={<IconSettings size={14} />}>Settings</Menu.Item>
                
                <Menu.Divider />

                <Menu.Label>Danger zone</Menu.Label>
                <Menu.Item icon={<IconSettings size={14} />}>SMTHG</Menu.Item>
                <Menu.Item color="red" icon={<IconTrash size={14} />}>Delete instance</Menu.Item>
            </Menu.Dropdown>
        </Menu>
    );
}