<template>
  <video autoplay loop muted class="video-content">
    <source v-bind:src="require('/src/assets/videos/videoBackground.mp4')" type="video/mp4">
  </video>
  <div class="main-content">
    <div class="bottle-container" v-for="(category,categoryIndex) in categories" :key="categoryIndex">
      <div class="bottle-top">
        <div class="bottle-title text-design text-with-font">
          <h1>{{ category.title }}</h1>
        </div>
      </div>
      <div class="bottle-content">
        <div class="cards scroll_bar_smooth" v-bind:id="category.category_id" @drop="dropCard" @dragover.prevent>
          <template v-for="(card,cardIndex) in cards" :key="cardIndex">
            <div class="card" draggable="true" @dragstart="event => onCardDrag(event,cardIndex)"
                 v-if="card.category_id === category.category_id">
              <div>
                <input class="input-title text-design text-with-font"
                       @blur="onFocusLost(card.id,card.title)"
                       @input="event => card.title = event.target.value"
                       type="text"
                       v-bind:value="card.title">
              </div>
            </div>
          </template>
        </div>
        <div class="add-div">
          <button class="add-button" @click="onAddCard(category.category_id)">
            <h2 class="text-design text-with-font">Ajouter</h2>
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script>

import {invoke} from "@tauri-apps/api";

export default {
  name: "Home-Page",
  data() {
    return {
      categories: [],
      cards: [],
      selectedCardIndex: null
    }
  },
  methods: {
    onAddCard: function (category_id) {
      invoke('add_card', {
        categoryId: category_id
      }).then((response) => {
        this.cards = []
        response.map(card => {
          this.cards.push({
            id: card.id,
            title: card.title,
            category_id: card.category_id
          })
        })
      })
    },
    dropCard: function (event) {
      if (event.target.id !== "") {
        this.cards[this.selectedCardIndex].category_id = event.target.id
        invoke('move_card', {
          id: this.cards[this.selectedCardIndex].id,
          categoryId: this.cards[this.selectedCardIndex].category_id
        }).then(response => {
          this.cards = []
          response.map(card => {
            this.cards.push({
              id: card.id,
              title: card.title,
              category_id: card.category_id
            })
          })
        })
      }
    },
    onCardDrag: function (event, cardIndex) {
      this.selectedCardIndex = cardIndex
      event.target.className = "card on-move"
    },
    onFocusLost: function (id, title) {
      invoke('set_card', {
        id: id,
        title: title
      }).then(response => {
        this.cards = []
        response.map(card => {
          this.cards.push({
            id: card.id,
            title: card.title,
            category_id: card.category_id
          })
        })
      })
    }
  },
  beforeMount() {
    invoke('get_cards').then(response => {
      this.cards = []
      response.map(card => {
        this.cards.push({
          id: card.id,
          title: card.title,
          category_id: card.category_id
        })
      })
    })
    invoke('get_categories').then(response => {
      this.categories = []
      response.map(card => {
        this.categories.push({
          id: card.id,
          title: card.title,
          category_id: card.category_id
        })
      })
    })
  }
}
</script>

<style scoped lang="scss">
.video-content {
  position: fixed;
  z-index: 0;
  min-height: 100%;
  min-width: 100%;
}

.main-content {
  position: absolute;
  z-index: 1;
  width: 80%;
  height: 80%;
  display: grid;
  justify-content: space-between;
  grid-template-columns: 25% 25% 25%;
  top: 20%;
  bottom: 10%;
  left: 10%;
  right: 10%;

  .bottle-container {


    .bottle-top {
      border-top-left-radius: 40%;
      border-top-right-radius: 40%;
      height: 20vh;
      box-shadow: 0 4px 30px rgba(0, 0, 0, 0.1);
      backdrop-filter: blur(1.3px);
      -webkit-backdrop-filter: blur(1.3px);
      background: rgba(255, 255, 255, 0.2);
      border-top: 1px solid rgba(255, 255, 255, 1);
      border-right: 1px solid rgba(255, 255, 255, 1);
      border-left: 1px solid rgba(255, 255, 255, 1);
      display: flex;
      justify-content: center;
      align-items: flex-end;

      .bottle-title {
        width: 100%;
        background-color: rgba(245, 245, 245, 0.8);
        display: flex;
        align-items: center;
        justify-content: center;
        text-transform: uppercase;
        height: 50%;

        h1 {
          margin: 0;
          text-align: center;
          color: black;
        }
      }
    }

    .bottle-top::before {
      position: absolute;
      top: -10vh;
      width: 30%;
      height: 10vh;
      content: "";
      box-shadow: 0 4px 30px rgba(0, 0, 0, 0.1);
      backdrop-filter: blur(1.3px);
      -webkit-backdrop-filter: blur(1.3px);
      background: rgba(255, 255, 255, 0.2);
      border-right: 1px solid rgba(255, 255, 255, 1);
      border-left: 1px solid rgba(255, 255, 255, 1);
    }

    .bottle-top::after {
      position: absolute;
      top: -13vh;
      width: 40%;
      height: 3vh;
      content: "";
      box-shadow: 0 4px 30px rgba(0, 0, 0, 0.1);
      backdrop-filter: blur(1.3px);
      -webkit-backdrop-filter: blur(1.3px);
      background: rgba(255, 255, 255, 0.2);
      border-right: 1px solid rgba(255, 255, 255, 1);
      border-left: 1px solid rgba(255, 255, 255, 1);
      border-radius: 10px;
    }

    .bottle-content {
      padding: 20px 10px;
      height: 50vh;
      box-shadow: 0 4px 30px rgba(0, 0, 0, 0.1);
      backdrop-filter: blur(1.3px);
      -webkit-backdrop-filter: blur(1.3px);
      background: rgba(255, 255, 255, 0.2);
      border-bottom-left-radius: 20px;
      border-bottom-right-radius: 20px;
      border-bottom: 1px solid rgba(255, 255, 255, 1);
      border-right: 1px solid rgba(255, 255, 255, 1);
      border-left: 1px solid rgba(255, 255, 255, 1);
      display: flex;
      justify-content: space-between;
      flex-direction: column;
      align-items: center;

      .cards {
        height: 100%;
        width: 100%;
        margin: 10px 0;
        min-height: 5vh;

        .card {
          margin: 10px;
          display: flex;
          justify-content: center;
          background-color: rgba(26, 46, 64, 0.7);
          color: white;
          min-height: 10vh;
          border-radius: 10px;

          .input-title {
            width: 100%;
            border: none;
            background-color: transparent;
            text-align: center;
            font-size: xx-large;
          }

          .input-title:focus {
            outline: none;
          }

          .on-move {
            background-color: cadetblue;
          }
        }
      }

      .add-div {
        width: 100%;
        display: flex;
        justify-content: center;

        .add-button {
          background-color: transparent;
          border: none;
          width: 100%;
          padding: 0;
        }

        .add-button:hover {
          transform: scale(1.2);
          transition: transform 1s;
        }
      }
    }
  }
}
</style>
