void setBuildStatus(String message, String state) {
  step([
      $class: "GitHubCommitStatusSetter",
      reposSource: [$class: "ManuallyEnteredRepositorySource", url: "https://github.com/khayyamsaleem/advent-of-code"],
      contextSource: [$class: "ManuallyEnteredCommitContextSource", context: "ci/jenkins/build-status"],
      errorHandlers: [[$class: "ChangingBuildStatusErrorHandler", result: "UNSTABLE"]],
      statusResultSource: [ $class: "ConditionalStatusResultSource", results: [[$class: "AnyBuildResult", message: message, state: state]] ]
  ]);
}

pipeline {
  agent any
  stages {
    stage('Build Intcode Service'){
      agent any
      environment {
        GITLAB_REGISTRY_CREDS = credentials('gitlab-registry')
      }
      steps {
        checkout scm
        script {
          sh("docker login -u $GITLAB_REGISTRY_CREDS_USR -p $GITLAB_REGISTRY_CREDS_PSW registry.gitlab.com")
          sh 'docker run --rm --privileged multiarch/qemu-user-static --reset -p yes'
          if (BRANCH_NAME == "master") {
            sh 'DOCKER_CLI_EXPERIMENTAL=enabled docker buildx rm builder || true'
            sh 'DOCKER_CLI_EXPERIMENTAL=enabled docker buildx create --name builder --driver docker-container --use'
            sh 'DOCKER_CLI_EXPERIMENTAL=enabled docker buildx inspect --bootstrap'
            sh 'DOCKER_CLI_EXPERIMENTAL=enabled docker buildx build ./2019/05/ -f ./2019/05/Dockerfile.arm --platform linux/arm -t registry.gitlab.com/khayyamsaleem/advent-of-code:intcode-service --load'
            sh 'docker push registry.gitlab.com/khayyamsaleem/advent-of-code:intcode-service'
          } else {
            echo 'Don\'t have a dev server yet, so just go ahead and push'
          }
        }
      }
    }
  }
  post {
    failure {
      script {
        if (BRANCH_NAME == 'master'){
          echo 'no images currently built'
          setBuildStatus("Build succeeded", "SUCCESS");
        } else {
          echo 'Something is wrong with develop???'
          setBuildStatus("Build failed", "FAILURE");
        }
      }
    }
    success {
      script {
        if (BRANCH_NAME == 'master'){
          echo 'rebuilt image successfully'
          setBuildStatus("Build succeeded", "SUCCESS");
        } else {
          echo 'Develop is good to merge!'
          setBuildStatus("Build succeeded", "SUCCESS");
        }
      }
    }
  }
}
