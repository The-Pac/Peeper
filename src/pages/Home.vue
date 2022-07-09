<template>
  <div class="main-video">
    <video>
      <source>
    </video>
  </div>
  <div class="main-content">
    <div class="column" v-for="(category,categoryIndex) in categories" :key="categoryIndex">
      <div class="column-title text-design text-with-font">
        <h1>{{ category.title }}</h1>
      </div>
      <div class="cards" v-bind:id="category.id" @drop="dropCard" @dragover.prevent>
        <template v-for="(card,cardIndex) in cards" :key="cardIndex">
          <div class="card" draggable="true" @dragstart="event => onCardDrag(event,cardIndex)"
               v-if="card.categoryId === category.id">
            <div>
              <input class="input-title text-design text-with-font"
                     @input="event => card.title = event.target.value"
                     type="text"
                     v-bind:value="card.title">
            </div>
            <div v-if="card.hasDate">
              <h2>{{ card.dateFrom + ' ' + card.dateTo }}</h2>
            </div>
          </div>
        </template>
      </div>
      <div class="add-div">
        <button class="add-button" @click.prevent="onClick">
          <h2 class="text-design text-with-font">Ajouter</h2>
        </button>
      </div>
    </div>
  </div>
</template>

<script>
import {invoke} from '@tauri-apps/api/tauri'

export default {
  name: "Home-Page",
  data() {
    return {
      categories: [
        {
          id: "todo",
          title: "à faire"
        },
        {
          id: "onprogress",
          title: "en cours"
        },
        {
          id: "finished",
          title: "terminé"
        },
      ],
      cards: [
        {
          title: "Course",
          hasDate: false,
          categoryId: "todo",
          dateFrom: "",
          dateTo: ""
        },
        {
          title: "Course",
          hasDate: false,
          categoryId: "todo",
          dateFrom: "",
          dateTo: ""
        },
        {
          title: "Course",
          hasDate: false,
          categoryId: "todo",
          dateFrom: "",
          dateTo: ""
        }
      ],
      selectedCardIndex: null
    }
  },
  mounted() {
    invoke('get-cards')
  },
  methods: {
    onClick: {},
    dropCard: function (event) {
      if (event.target.id !== "") {
        this.cards[this.selectedCardIndex].categoryId = event.target.id
        invoke('set-card')
      }
    },
    onCardDrag: function (event, cardIndex) {
      this.selectedCardIndex = cardIndex
      event.target.className = "card on-move"
    }
  }
}
</script>

<style scoped lang="scss">
.main-video {
  position: absolute;
  z-index: 0;
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
}

.main-content {
  position: absolute;
  z-index: 1;
  width: 80%;
  height: 80%;
  display: grid;
  justify-content: space-between;
  grid-template-columns: 25% 25% 25%;
  top: 10%;
  bottom: 10%;
  left: 10%;
  right: 10%;

  .column {
    padding: 10px;
    height: fit-content;
    background-color: rgba(26, 46, 64, 0.7);
    border: 5px solid #22A2F2;
    display: flex;
    flex-direction: column;
    align-items: center;

    .column-title {
      width: 100%;
      display: flex;
      align-items: center;
      justify-content: center;
      text-transform: uppercase;
      height: 20%;
      border-bottom: 5px solid #22A2F2;

      h1 {
        margin: 0;
        text-align: center;
      }
    }

    .cards {
      width: 100%;
      margin: 10px 0;
      min-height: 5vh;

      .card {
        margin: 10px 0;
        display: flex;
        justify-content: center;
        background-color: rgba(26, 46, 64, 1);
        color: white;
        min-height: 10vh;

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
      height: 20%;
      display: flex;
      justify-content: center;

      .add-button {
        background-color: #22A2F2;
        border: none;
        width: 100%;
        height: 20%;
        padding: 0;
      }
    }
  }
}
</style>
