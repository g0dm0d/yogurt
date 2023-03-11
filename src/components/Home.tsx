import {
    Box, Card, createStyles, Grid, Modal,
} from '@mantine/core';
import { useHover } from '@mantine/hooks';
import { IconSquarePlus } from '@tabler/icons-react';
import { useState, useEffect } from 'react';
import { InstanceCard } from './ui/instanceCard';
import { AddInstance } from './popups/AddInstance';
import { invoke } from '@tauri-apps/api';

interface Instance {
    name: string;
    version: string;
    gameType: string;
}

async function getInstances(setInstances: React.Dispatch<React.SetStateAction<Instance[]>>) {
    try {
        const response = await invoke<Instance[]>('get_all_instances');
        console.log(response);
        setInstances(response);
    } catch (error) {
        console.error(error);
    }
}

export function Home() {
    const { hovered, ref } = useHover();

    const useStyles = createStyles((theme) => ({
        card: {
            width: '216px',
            height: '164px',
            border: '2px dashed',
            transition: '300ms',
            borderColor: hovered ? theme.colors.dark[1] : theme.colors.dark[3],
            display: 'flex',
            justifyContent: 'center',
            alignItems: 'center',
            cursor: 'pointer',
        },

        addIcon: {
            transition: '300ms',
            position: 'absolute',
        },
    }));
    const { classes } = useStyles();
    const [openModal, setOpenModal] = useState(false);

    const [instances, setInstances] = useState<Instance[]>([]);

    useEffect(() => {
        getInstances(setInstances);
    }, []);

    const instancesList = instances.map((instance) =>
        <Grid.Col span='content' key={instance.name}>
            <InstanceCard
                key={instance.name}
                name={instance.name}
                version={instance.version}
                gameType={instance.gameType}
            />
        </Grid.Col>
    );

    return (
        <Box sx={{
            display: 'flex', justifyContent: 'start', alignItems: 'start',
            height: '100%', width: '100%', padding: '80px', gap: '32px'
        }}>
            <Modal opened={openModal} onClose={() => setOpenModal(false)} title='Create Instance'>
                <AddInstance />
            </Modal>
            <Grid justify="flex-start">
                {instancesList}
                <Grid.Col span='content'>
                    <Card ref={ref} p="lg" className={classes.card} onClick={() => setOpenModal(true)}>
                        <IconSquarePlus size={hovered ? '48px' : '36px'} stroke={1} className={classes.addIcon} />
                    </Card>
                </Grid.Col>
            </Grid>
        </Box>
    );
}