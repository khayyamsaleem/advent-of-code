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
    stage('Deploy'){
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
                sh 'docker build ./2019/05/ -t registry.gitlab.com/khayyamsaleem/advent-of-code:intcode'
                sh 'docker push registry.gitlab.com/khayyamsaleem/advent-of-code:intcode'
                sh 'docker stop $(docker ps -a | grep advent-of-code | awk \'{ print $1 }\') || true'
                sh 'docker rm $(docker ps -a | grep advent-of-code | awk \'{ print $1 }\') || true'
                sh 'docker rmi $(docker images | grep advent-of-code | awk \'{ print $3 }\') || true'
                sh 'docker-compose -f ./2019/05/docker-compose-prod.yml up --build -d'
                echo 'successfully deployed'
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
        } else {
          echo 'Something is wrong with develop???'
        }
        setBuildStatus("Build failed", "FAILURE");
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
