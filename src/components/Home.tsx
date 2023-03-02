import {
    Box, Card, createStyles, Modal,
} from '@mantine/core';
import { useHover } from '@mantine/hooks';
import { IconSquarePlus } from '@tabler/icons-react';
import { useState } from 'react';
import { InstanceCard } from './ui/instanceCard';
import { AddInstance } from './popups/AddInstance';

export function Home() {
    const { hovered, ref } = useHover();

    const useStyles = createStyles((theme) => ({
        card: {
            minWidth: '200px',
            minHeight: '164px',
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

    return (
        <Box sx={{
            display: 'flex', justifyContent: 'start', alignItems: 'start',
            height: '100%', width: '100%', padding: '80px', gap: '32px'
        }}>
            <Modal opened={openModal} onClose={() => setOpenModal(false)} title='Create Instance'>
                <AddInstance />
            </Modal>
            <InstanceCard title='minecraft' version='1.19.2' type='Fabric' />
            <InstanceCard title='minecraft 1.18.2' version='1.18.2' type='Fabric' />
            <Card ref={ref} className={classes.card} onClick={() => setOpenModal(true)}>
                <IconSquarePlus size={hovered ? '48px' : '36px'} stroke={1} className={classes.addIcon} />
            </Card>
        </Box>
    );
}