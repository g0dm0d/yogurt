/* eslint-disable no-unneeded-ternary */
/* eslint-disable @typescript-eslint/strict-boolean-expressions */
/* eslint-disable react/react-in-jsx-scope */
/* eslint-disable @typescript-eslint/explicit-function-return-type */
import { Box } from '@mantine/core'
import { useEffect, useState } from 'react'
import steveHead from './icons/SteveHead.png'

interface PlayerHeadProps {
  nickname?: string
  size?: number
}

export function PlayerHead ({ nickname, size = 50 }: PlayerHeadProps) {
  const [head, setHead] = useState<string>()
  // const [uuid, setUuid] = useState((Boolean(nickname)) || undefined)

  const fetchImage = async () => {
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
            <img src={head ? head : steveHead} width={size} height={size} />
        </Box>
  )
}
