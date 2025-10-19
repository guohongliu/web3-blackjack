<template>
  <div className = "flex flex-col gap-2 items-center justify-center h-screen bg-gray-300">
    <h1 className = "text-3xl hold">Welcome To Web3 Game Black Jack</h1>
    <h2 className = "text-2xl hold">Message: Player wins/ dealer wins: BlackJack / bust!</h2>
    <div>
      <h2>Dealer's hand</h2>
      <div className = "flex flex-row gap-2">
        <div v-for="(card, index) in dealerHand" :key="index" className = "w-32 h-42 border-black bg-white rounded-md flex flex-col justify-between">
          <p className = "text-black self-start p-2">{{ card.rank }}</p>
          <p className = "self-center p-2 text-3xl">{{ card.suit }}</p>
          <p className = "text-black self-end p-2">{{ card.rank }}</p>
        </div>
      </div>
    </div>
    <div>
      <h2>Player's hand</h2>
      <div className = "flex flex-row gap-2">
      <div v-for="(card, index) in playerHand" :key="index" className = "w-32 h-42 border-black bg-white rounded-md flex flex-col justify-between">
          <p className = "text-black self-start p-2">{{ card.rank }}</p>
          <p className = "self-center p-2 text-3xl">{{ card.suit }}</p>
          <p className = "text-black self-end p-2">{{ card.rank }}</p>
        </div>
      </div>
    </div>
    <div className = "flex flex-row gap-2 mt-4">
      <button className = "bg-amber-300 rounded-md p-2">Hit</button>
      <button className = "bg-amber-300 rounded-md p-2">Stand</button>
      <button className = "bg-amber-300 rounded-md p-2">Reset</button>
    </div>
  </div>
</template>

<script setup>
  import { onMounted, ref } from "vue";
  import api from "@/utils/fetchRequest";

  const dealerHand = ref([]);
  const playerHand = ref([]);
  const deck = ref([]);

  const API_BASE_URL = "http://localhost:8080/api";
  const newGame = async () => {
    // debugger;
    api.post(`${API_BASE_URL}/game/new`).then(res => {
      if (res.status === 200) {
        dealerHand.value.push(res.data.dealer_hand);
        playerHand.value.push(res.data.player_hand);
        deck.value.push(res.data.deck);
      }
    })
  }

  onMounted(() => newGame())

</script>
