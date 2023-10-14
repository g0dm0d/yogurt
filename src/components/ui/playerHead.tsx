import { Box } from '@mantine/core'
import { useEffect, useState } from 'react'
import steveHead from './icons/SteveHead.png'

interface PlayerHeadProps {
  nickname?: string
  size?: number
}

export function PlayerHead ({ nickname, size = 50 }: PlayerHeadProps): JSX.Element {
  const [head, setHead] = useState<string>()
  // const [uuid, setUuid] = useState((Boolean(nickname)) || undefined)

  const fetchImage = async (): Promise<void> => {
    if ((nickname != null) && (head == null)) {
      const response = await fetch(`https://mc-heads.net/avatar/${nickname}/${size}`)
      const data = await response.blob()
      const imageHead = URL.createObjectURL(data)
      // console.log(imageHead);
      setHead(imageHead)
    }
  }

  useEffect(() => {
    if (head == null) {
      void fetchImage()
    }
  }, [nickname])

  return (
        <Box>
            <img src={head ?? steveHead} width={size} height={size} />
        </Box>
  )
}
