pipeline {
    agent none
    stages {
        stage('Checkout') {
            agent any
            steps {
                checkout([
                    $class: 'GitSCM',
                    branches: [[name: env.GIT_BUILD_REF]],
                    userRemoteConfigs: [[url: env.GIT_REPO_URL, credentialsId: env.CREDENTIALS_ID]]
                ])
            }
        }
        stage('Install modules & Build') {
            agent {
                docker {
                    image 'rust:1.47'
                }
            }
            steps {
                sh 'apt-get install make cmake'
                sh 'cargo build --release'
                sh 'ls ./target/release'
            }
        }
        stage('Pack docker image') {
            agent any
            steps {
                script {
                    def myImage = docker.build('hwj-zone/hwj-zone-server:${CI_BUILD_NUMBER}')
                    docker.withRegistry('https://ccr.ccs.tencentyun.com', '7ed95a1d-0702-4321-a8db-f238a9c01ec0') {
                        myImage.push(env.CI_BUILD_NUMBER)
                    }
                }
            }
        }
    }
}