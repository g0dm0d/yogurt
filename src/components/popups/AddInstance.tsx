import {
  Box,
  Button,
  Center,
  Flex,
  SegmentedControl,
  Select,
  Image,
  TextInput,
  Checkbox
} from '@mantine/core'
import { IconChevronDown } from '@tabler/icons-react'
import { invoke } from '@tauri-apps/api/tauri'
import { useState, useEffect } from 'react'
import FabricIcon from '../ui/icons/FabricIcon.svg'
import MinecraftIcon from '../ui/icons/MinecraftIcon.svg'
import { showNotification } from '@mantine/notifications'

interface Version {
  label: string
  value: string
}

export function AddInstance({ setCreating }: { setCreating: React.Dispatch<React.SetStateAction<boolean>> }): JSX.Element {
  async function createInstance(name: string, version: string | undefined, type: string, url?: string | null): Promise<void> {
    if (url == null) {
      void fetch('https://launchermeta.mojang.com/mc/game/version_manifest_v2.json')
        .then(async response => await response.json())
        .then(
          (result) => {
            for (let i = 0; i < result.versions.length; i++) {
              if (result.versions[i].id === version) {
                url = result.versions[i].url
              }
            }
            name = name + ' copy'
          },
          (error) => {
            console.error(error)
            showNotification({
              title: 'Error',
              message: error,
              color: 'red'
            })
          }
        )
        .then(() => {
          void getMinecraft()
        })
    } else {
      void getMinecraft()
    }

    async function getMinecraft(): Promise<void> {
      setCreating(true)
      await invoke('get_minecraft', {
        url,
        id: version,
        name,
        javaArgs: '-Xmx4G',
        fabric: type === 'fabric'
      })
        .then(async (response) => {
          console.log(java)
          if (java) {
            await invoke('install_java', {
              instanceName: name
            })
          }
          console.log(response)
          setCreating(false)
        })
        .catch((error) => {
          console.log(error)
          setCreating(false)
          showNotification({
            title: 'Error occurred during installation process',
            message: error,
            color: 'red'
          })
        })
    }
  }

  const [type, setType] = useState('minecraft')
  const [name, setName] = useState('')

  const [versions] = useState<Version[]>([])
  const [value, setValue] = useState<string | null>(null)
  const label = versions.find((item) => item.value === value)?.label

  const [fabricVersions] = useState<Version[]>([])
  const [fabricValue, setFabricValue] = useState<string | null>(null)
  const fabricLabel = fabricVersions.find((item) => item.value === fabricValue)?.label

  const [java, setJava] = useState(true)

  const [, setLoading] = useState(false)
  async function getDefaultVersions(): Promise<void> {
    setLoading(true)
    fetch('https://launchermeta.mojang.com/mc/game/version_manifest_v2.json')
      .then(async response => await response.json())
      .then(
        (result) => {
          setLoading(false)
          for (let i = 0; i < result.versions.length; i++) {
            const versionObj: Version = { label: result.versions[i].id, value: result.versions[i].url }
            versions.push(versionObj)
            // setVersions([...versions, versionObj]);
          }
          void getFabcricVersions()
        },
        (error) => {
          setLoading(false)
          console.error(error)
          setValue('error')
        }
      )
  }

  async function getFabcricVersions(): Promise<void> {
    setLoading(true)
    fetch('https://meta.fabricmc.net/v2/versions/game')
      .then(async response => await response.json())
      .then(
        (result) => {
          setLoading(false)
          for (let i = 0; i < result.length; i++) {
            const findVersion = versions.find((version) => version.label === result[i].version)?.value
            const fabricVersionObj: Version = { label: result[i].version, value: findVersion ?? '' }
            fabricVersions.push(fabricVersionObj)
            // setFabricVersions([...fabricVersions, versionObj]);
          }
        },
        (error) => {
          setLoading(false)
          console.error(error)
          setValue('error')
        }
      )
  }

  useEffect(() => {
    void getDefaultVersions()
  }, [])

  return (
    <form onSubmit={(e) => {
      e.preventDefault()
      if (type === 'minecraft') {
        void createInstance(name, label, type, value)
      }
      if (type === 'fabric') {
        void createInstance(name, fabricLabel, type, fabricValue)
      }
    }}>
      <Box sx={{ display: 'flex', alignItems: 'center', minHeight: '30vh', height: '100%', width: '100%' }}>
        <Flex direction='column' gap='lg' justify='space-between' sx={{ height: '100%', width: '100%' }}>
          <TextInput
            placeholder="Name"
            label="Instance name"
            value={name}
            onChange={(event) => { setName(event.currentTarget.value) }}
            required
          />
          <SegmentedControl
            value={type}
            onChange={(value: 'minecraft' | 'fabric') => { setType(value) }}
            data={[
              {
                value: 'minecraft',
                label: (
                  <Center>
                    <img width={24} height={24} src={MinecraftIcon} alt="Minecraft Icon" />
                    <Box ml={10}>Minecraft</Box>
                  </Center>
                )
              },
              {
                value: 'fabric',
                label: (
                  <Center>
                    <Image width={24} height={24} src={FabricIcon} alt="Fabric Icon" />
                    <Box ml={10}>Fabric</Box>
                  </Center>
                )
              }
            ]}
          />
          <Select
            data={versions}
            value={value}
            onChange={setValue}
            description="Version"
            placeholder='Version'
            searchable
            nothingFound="Error"
            rightSection={<IconChevronDown size="1rem" />}
            transition='fade'
            transitionDuration={200}
            required={type === 'minecraft'}
            sx={{ display: type === 'minecraft' ? 'visible' : 'none' }}
          />
          <Select
            data={fabricVersions}
            value={fabricValue}
            onChange={setFabricValue}
            description="Fabric version"
            placeholder='Version'
            searchable
            nothingFound="Error"
            rightSection={<IconChevronDown size="1rem" />}
            transition='fade'
            transitionDuration={200}
            required={type === 'fabric'}
            sx={{ display: type === 'fabric' ? 'visible' : 'none' }}
          />
          <Checkbox label="Download Java" checked={java} onChange={() => { setJava(!java) }} />
          <Button type='submit' variant='outline' >
            Create
          </Button>
        </Flex>
      </Box >
    </form >
  )
}
