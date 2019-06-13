import React, { useState, useRef, useEffect } from 'react'
import { Mutation } from 'react-apollo'
import { withRouter } from 'react-router-dom'
import { Formik } from 'formik'
import randomstring from 'randomstring'

import {
  CREATE_WORLD_MUTATION,
  CREATE_WORLD_SCHEMA
} from '../../../../../lib/graphql'
import { Hint } from '../../../../Utils'
import classes from './CreateNewWorld.module.css'
import sharedStyles from '../../../../../containers/sharedStyles.module.css'

const CreateNewWorld = withRouter(({ history }) => {
  const gamemodeDictionary = {
    SURVIVAL: {
      title: 'Survival',
      description:
        'Search for resources, crafting, gain levels, health and hunger'
    },
    CREATIVE: {
      title: 'Creative',
      description:
        'Unlimited resources, free flying and destroy blocks instantly'
    }
  }
  const gamemodes = ['SURVIVAL', 'CREATIVE']
  const [gamemode, setGamemode] = useState(0)

  const worldNameInput = useRef()

  useEffect(() => {
    worldNameInput.current.focus()
  })

  return (
    <Mutation
      mutation={CREATE_WORLD_MUTATION}
      onCompleted={data => {
        const {
          createWorld: { id }
        } = data

        history.push(`/game/minecraft/${id}`)
      }}
      onError={error => console.error(error)}
    >
      {(createWorld, { loading }) =>
        loading ? (
          <Hint text="Generating" />
        ) : (
          <Formik
            initialValues={{ name: 'New World', seed: '' }}
            validationSchema={CREATE_WORLD_SCHEMA}
            onSubmit={(values, { setSubmitting }) => {
              createWorld({
                variables: {
                  name: values.name,
                  seed: !!values.seed ? values.seed : randomstring.generate(),
                  gamemode: gamemodes[gamemode]
                }
              })
              setSubmitting(false)
            }}
            render={({
              values,
              errors,
              touched,
              handleChange,
              handleBlur,
              handleSubmit,
              isSubmitting
            }) => {
              return (
                <form
                  onSubmit={handleSubmit}
                  style={{ display: 'flex', flexDirection: 'column' }}
                  className={classes.wrapper}
                >
                  <h1 className={classes.title}>Create New World</h1>

                  <div className={classes.inputField}>
                    <p>World Name</p>
                    <input
                      ref={worldNameInput}
                      autoComplete="off"
                      id="name"
                      name="name"
                      value={values.name}
                      label="name"
                      onChange={handleChange}
                      onBlur={handleBlur}
                      placeholder="Name"
                    />
                    <p>Will be saved in: {values.name}</p>
                  </div>

                  <div className={classes.inputField}>
                    <p>Seed for the world generator</p>
                    <input
                      autoComplete="off"
                      id="seed"
                      name="seed"
                      value={values.password}
                      type="seed"
                      label="Seed"
                      onChange={handleChange}
                      onBlur={handleBlur}
                      placeholder="Seed"
                    />
                    <p>Leave blank for random seed</p>
                  </div>

                  <div className={classes.gamemode}>
                    <button
                      className={sharedStyles.button}
                      type="button"
                      onClick={() => {
                        setGamemode((gamemode + 1) % gamemodes.length)
                      }}
                    >
                      Game Mode: {gamemodeDictionary[gamemodes[gamemode]].title}
                    </button>
                    <p>{gamemodeDictionary[gamemodes[gamemode]].description}</p>
                  </div>

                  <div className={classes.finalButts}>
                    <button
                      className={sharedStyles.button}
                      type="submit"
                      disabled={
                        !values.name ||
                        isSubmitting ||
                        !!(errors.name && touched.name) ||
                        !!(errors.seed && touched.seed)
                      }
                    >
                      Create New World
                    </button>
                    <button
                      className={sharedStyles.button}
                      type="button"
                      onClick={() => history.goBack()}
                    >
                      Cancel
                    </button>
                  </div>
                </form>
              )
            }}
          />
        )
      }
    </Mutation>
  )
})

export default CreateNewWorld
