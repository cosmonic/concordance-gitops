import React, { useState } from "react";
import { ChakraProvider, Box, Grid, Center, Button, Text } from "@chakra-ui/react";
import { Logo } from "./Logo";
import { ColorModeSwitcher } from './ColorModeSwitcher';

const FRUIT_JOKE_API = "/joke";

function App() {
  const [joke, setJoke] = useState("");
  const [punchlineVisible, setPunchlineVisible] = useState(false);
  const [punchline, setPunchline] = useState("");

  const fetchJoke = async () => {
    try {
      const response = await fetch(FRUIT_JOKE_API);
      const data = await response.text();
      const [jokeText, punchlineText] = data.split("? ");
      setJoke(jokeText);
      setPunchline(punchlineText);
      setPunchlineVisible(false); // Hide punchline after fetching new joke
    } catch (error) {
      console.error("Error fetching joke:", error);
    }
  };

  const revealPunchline = () => {
    setPunchlineVisible(true);
  };

  return (
    <ChakraProvider>
      <Box textAlign="center" fontSize="xl">
        <Grid p="3">
          <ColorModeSwitcher justifySelf="flex-end" />
          <Center height="90vh">
            <Box width="100%" textAlign="center">
              {/* TODO: Make laugh lemon minified, it's beeg */}
              <Logo src="laugh-lemon.png" alt="Fruit Logo" maxW="400px" mx="auto" mb="4" laugh={punchlineVisible.toString()} />
              <Button onClick={fetchJoke} colorScheme="blue" mb="4">
                Generate Fruit Joke
              </Button>
              {!punchlineVisible && joke && (
                <Button onClick={revealPunchline} colorScheme="green" ml="2" mb="4">
                  Reveal Punchline
                </Button>
              )}
              {joke && (
                <Text fontSize="2xl" fontWeight="bold" mb="2">
                  {joke}?
                </Text>
              )}
              {punchlineVisible && (
                <Text color="gray.200" fontSize="xl">
                  {punchline}
                </Text>
              )}
            </Box>
          </Center>
        </Grid>
      </Box>
    </ChakraProvider>

  );
}

export default App;
