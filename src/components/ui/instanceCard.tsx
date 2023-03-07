import {
    createStyles,
    Card,
    Image,
    Text,
    Box,
    Popover,
} from '@mantine/core';
import { useHover } from '@mantine/hooks';
import { IconAdjustmentsHorizontal, IconPlayerPlay } from '@tabler/icons-react';
import { invoke } from '@tauri-apps/api';
import { useState } from 'react';
import bg from '/bg.png';

interface InstanceCardProps {
    title: string;
    version: string;
    type: string;
}

async function startInstance(instance: string) {
    if (instance) {
        try {
            await invoke('run_minecraft', {
                username: '',
                uuid: '',
                token: '',
                instance: instance
            });
            console.log(Response);
        } catch (error) {
            console.error(error);
        }
    }
}

export function InstanceCard({ title, version, type }: InstanceCardProps) {
    const { hovered, ref } = useHover();
    const useStyles = createStyles((theme) => ({
        card: {
            minWidth: '200px',
            minHeight: '160px',
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

        title: {
            color: theme.colorScheme === 'dark' ? theme.colors.dark[1] : theme.colors.gray[1],
            fontSize: theme.fontSizes.lg,
            transition: '300ms',
            position: 'absolute',
            opacity: hovered ? 0 : 1,
        },
    }));
    const { classes } = useStyles();
    const [openPopover, setOpenPopover] = useState(false)
    return (
        <Popover opened={openPopover} onChange={setOpenPopover}>
            <Card p="lg" className={classes.card}>
                <Card.Section>
                    <Box ref={ref} display='flex' onClick={() => startInstance(title)}
                        sx={{
                            justifyContent: 'center',
                            alignItems: 'center',
                            cursor: 'pointer'
                        }} >
                        <Image className={classes.image} src={bg} alt={title} height={100} />
                        <Text size="sm" weight={700} className={classes.title}>
                            {title}
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
                        <IconAdjustmentsHorizontal cursor='pointer' onClick={() => setOpenPopover(true)} />
                    </Box>
                </Card.Section>
            </Card>
            <Popover.Dropdown />
        </Popover>
    );
}